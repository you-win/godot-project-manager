class_name TranslationHelper
extends RefCounted

const COMMENT_CHARS := ";;"
const ESCAPED_QUOTE := "\\\""
const FIXED_QUOTE := "\""

const TRANSLATION_EXTENSION := ".vpptx"

var logger := Logger.new("TranslationHelper")

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

static func _is_valid_ending(text: String) -> bool:
	return text.ends_with(FIXED_QUOTE) and not text.ends_with(ESCAPED_QUOTE)

static func _clean_message(text: String) -> String:
	return text.substr(1, text.length() - 2)

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#

func load_translation(locale: String, text: String) -> Result:
	logger.debug("Loading translation for locale %s" % locale)
	
	var translation := Translation.new()
	translation.locale = locale
	
	# Attempt to normalize line endings
	text = text.replace("\r", "")
	
	var current_key := ""
	var current_message := ""
	for line in text.split("\n"):
		line = line.substr(0, line.find(COMMENT_CHARS)).strip_edges()
		
		var split := line.split("=", true, 1)
		if split.size() < 2:
			current_message += "\n%s" % line.replace(ESCAPED_QUOTE, FIXED_QUOTE)
			continue
		
		# Spaces are valid before and after the "="
		for i in split.size():
			split[i] = split[i].strip_edges()
		
		if not current_key.is_empty():
			var final_message := current_message.strip_edges()
			if not _is_valid_ending(final_message):
				current_message += "\n%s" % line.replace(ESCAPED_QUOTE, FIXED_QUOTE)
				continue
			else:
				final_message = _clean_message(final_message)
				translation.add_message(current_key, final_message)
				
				logger.debug("Added translation: %s - %s" % [
					current_key, final_message
				])
				
				current_key = ""
				current_message = ""
		
		current_key = split[0]
		current_message = split[1].replace(ESCAPED_QUOTE, FIXED_QUOTE)
		if not current_message.begins_with(FIXED_QUOTE):
			logger.error("Translation messages must start with a \" character, skipping: %s" %
				current_key)
			current_key = ""
			current_message = ""
			continue
	
	var final_message := current_message.strip_edges()
	if not _is_valid_ending(final_message):
		logger.error("Translation messages must end with a \" character, skipping: %s" %
			current_key)
	else:
		translation.add_message(current_key, _clean_message(final_message))
	
	logger.debug("Loaded %d translations" % translation.get_message_count())
	
	TranslationServer.add_translation(translation)
	
	logger.debug("Finished loading translation for locale %s" % locale)
	
	return Safely.ok()

func scan(path: String) -> Result:
	logger.debug("Scanning %s" % path)
	
	var dir := DirAccess.open(path)
	dir.include_hidden = false
	dir.include_navigational = false
	if dir == null:
		return Safely.err(Error.Code.CANNOT_OPEN_DIRECTORY, path)
	
	dir.list_dir_begin()
	
	var file_name := "start"
	while not file_name.is_empty():
		file_name = dir.get_next()
		
		if dir.current_is_dir():
			continue
		if not file_name.ends_with(TRANSLATION_EXTENSION):
			continue
		
		var file_path := "%s/%s" % [path, file_name]
		var file := FileAccess.open(file_path, FileAccess.READ)
		if file == null:
			return Safely.err(Error.Code.CANNOT_OPEN_FILE, file_path)
		
		logger.debug("Reading translation file %s" % file_path)
		
		load_translation(file_name.trim_suffix(TRANSLATION_EXTENSION), file.get_as_text())
		
		logger.debug("Finished reading translation file %s" % file_path)
	
	logger.debug("Finished scanning %s" % path)
	
	return Safely.ok()
