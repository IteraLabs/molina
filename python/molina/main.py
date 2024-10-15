from molina import read_pdf

def main():

    file_route = "/Users/franciscome/git/iteralabs/molina/"
    file_folder = "knowledge/behavioral_economics/"
    file_name = "Advances-Prospect-Kahneman-Tversky-1992.pdf"
    
    result_pdf = read_pdf(
        in_file=file_route + file_folder + file_name)

    print(result_pdf)

if __name__ == "__main__":
    main()

