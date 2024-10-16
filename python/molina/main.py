from molina import extract_content

def main():

    wd_folder = "/Users/franciscome/git/iteralabs/molina"
    in_folder = "/knowledge"
    in_subfolder = "/conference_icml"
    in_file = "/alon22a.pdf"
    
    result_pdf = extract_content(
        input_file=wd_folder + in_folder + in_subfolder + in_file)

    print(result_pdf)

if __name__ == "__main__":
    main()

