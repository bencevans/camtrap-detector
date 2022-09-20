import { useDropzone } from "react-dropzone";
import React from "react";
import styled from "styled-components";

const getColor = (props) => {
  if (props.isDragAccept) {
    return "#00e676";
  }
  if (props.isDragReject) {
    return "#ff1744";
  }
  if (props.isFocused) {
    return "#2196f3";
  }
  return "#ccc";
};

const Container = styled.div`
  height: 100%;
  flex-grow: 1;
  flex: 1;
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 20px;
  border-width: 4px;
  border-radius: 8px;
  border-color: ${(props) => getColor(props)};
  border-style: dashed;
  // background-color: #fafafa;
  color: #bdbdbd;
  outline: none;
  transition: border 0.24s ease-in-out;
  text-align: center;
  height: 100%;
`;

export default function MyDropzone(props) {
  const { getRootProps, getInputProps, isFocused, isDragAccept, isDragReject } =
    useDropzone({
      // accepts folders
      accept: "application/x-moz-file",
    });

  return (
    <>
      <Container {...getRootProps({ isFocused, isDragAccept, isDragReject })}>
        <input {...getInputProps()} />
        <p style={{ margin: "auto" }}>
          Drag 'n' drop your Camera Trap Image Folder
        </p>
      </Container>
      <form style={{
        margin: "auto",
        paddingTop: "10px",
      }}>
        <label 
        style={{
          margin: "auto",
          paddingRight: "10px",
          fontSize: "12px",
        
        }}
        >
          Recursively search subfolders:
          <input
            name="searchSubfolders"
            type="checkbox"
            checked={props.searchSubfolders}
            onChange={props.handleSearchSubfoldersChange}
            style={{
              marginLeft: "10px",
            }}
            />
        </label>
      </form>
    </>
  );
}
