
void __fastcall FUN_00610900(int *param_1)

{
  POINT pt;
  BOOL BVar1;
  int iVar2;
  int unaff_EBX;
  int iVar3;
  tagPOINT tStack_30;
  int iStack_28;
  RECT RStack_24;
  int iStack_14;
  undefined1 auStack_10 [4];
  int iStack_c;
  
  if (param_1[0x27] != 0) {
    GetCursorPos(&tStack_30);
    MapWindowPoints((HWND)0x0,(HWND)param_1[6],&tStack_30,1);
    if ((tStack_30.x != param_1[0x28]) && (tStack_30.y != param_1[0x29])) {
      (**(code **)(*param_1 + 0x30))(auStack_10);
      RStack_24.right = param_1[0xc];
      RStack_24.bottom = param_1[0xd];
      RStack_24.left = 0;
      RStack_24.top = 0;
      pt.y = tStack_30.x;
      pt.x = unaff_EBX;
      BVar1 = PtInRect(&RStack_24,pt);
      if (BVar1 == 0) {
        iVar3 = param_1[0x28];
        iVar2 = param_1[0x29];
      }
      else {
        iVar3 = iStack_14;
        iVar2 = iStack_28;
        if ((iStack_14 <= unaff_EBX) &&
           (iVar3 = unaff_EBX, iVar2 = tStack_30.x, iStack_c < unaff_EBX)) {
          iVar3 = iStack_c;
          iVar2 = iStack_28;
        }
      }
      iStack_28 = iVar2;
      iVar3 = (((iVar3 - *(int *)(param_1[0x35] + 0x30)) - param_1[0x2a]) * param_1[0x37]) /
              (iStack_c - iStack_14) - param_1[0x38];
      if (((int *)param_1[0x26] != (int *)0x0) &&
         (iVar2 = (**(code **)(*(int *)param_1[0x26] + 0x28))(iVar3), iVar2 != 0)) {
        FUN_0060b3d0(iVar3);
      }
    }
  }
  return;
}

