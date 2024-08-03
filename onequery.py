from pymongo import MongoClient

def search_word_in_collection(username, password, database_name, collection_name, search_word, columns=[]):
    host = 'localhost'
    port = 27017
    uri = f"mongodb://{username}:{password}@{host}:{port}"
    # Connect to MongoDB
    client = MongoClient(uri)
    db = client[database_name]
    collection = db[collection_name]

    # Create a filter to search for the word in specified columns
    filters = []
    for column in columns:
        filters.append({column: {"$regex": search_word, "$options": "i"}})

    # If no columns are specified, search in all columns
    if not filters:
        filters = [{"$or": [{key: {"$regex": search_word, "$options": "i"}} for key in collection.find_one().keys()]}]

    # Search for the word in the collection
    result = collection.find({"$or": filters})

    # Print the matched documents
    for document in result:
        print(document)

# Example usage
username = 'root'
password = 'onecrawlrootpass'
database_name = "crawler"
collection_name = "page_informations"
search_word = "australia"
columns = ["content_text", "description"]  # Specify the columns to search in (leave empty to search in all columns)
search_word_in_collection(username, password, database_name, collection_name, search_word, columns)
