function celldetect(image)
    load("1ch_rcnn.mat","detector");
    img = imread(image);
    [bbox, score, labels] = detect(detector, img);

    detectedImg = insertObjectAnnotation(img,'rectangle',bbox,cellstr(labels));
    imwrite(detectedImg,"../oimg/Output_001.jpg")
    
end

