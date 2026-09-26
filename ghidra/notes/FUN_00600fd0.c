
void FUN_00600fd0(HDC param_1,LPCSTR param_2,LPRECT param_3,COLORREF param_4,COLORREF param_5,
                 UINT param_6,int param_7,int param_8)

{
  tagRECT local_10;
  
  local_10.left = param_3->left + param_7;
  local_10.right = param_3->right + param_7;
  local_10.bottom = param_3->bottom + param_7;
  local_10.top = param_3->top + param_7;
  SetTextColor(param_1,param_5);
  SetBkMode(param_1,1);
  DrawTextA(param_1,param_2,param_8,&local_10,param_6);
  SetTextColor(param_1,param_4);
  DrawTextA(param_1,param_2,param_8,param_3,param_6);
  return;
}

