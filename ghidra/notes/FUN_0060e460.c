
LRESULT FUN_0060e460(int param_1,UINT param_2,WPARAM param_3,LPARAM param_4)

{
  int iVar1;
  int iVar2;
  LRESULT LVar3;
  HDC hdc;
  HGDIOBJ pvVar4;
  HGDIOBJ pvVar5;
  uint uVar6;
  uint uVar7;
  code *pcVar8;
  undefined1 local_40 [52];
  HGDIOBJ pvStack_c;
  HGDIOBJ pvStack_8;
  HGDIOBJ pvStack_4;
  
  if (param_2 == 0xf) {
    hdc = BeginPaint(*(HWND *)(param_1 + 0x18),(LPPAINTSTRUCT)local_40);
    pcVar8 = SelectObject_exref;
    if (*(HGDIOBJ *)(param_1 + 0xb0) != (HGDIOBJ)0x0) {
      pvVar4 = SelectObject(hdc,*(HGDIOBJ *)(param_1 + 0xb0));
      pvVar5 = GetStockObject(8);
      pvVar5 = SelectObject(hdc,pvVar5);
      Rectangle(hdc,0,0,*(int *)(param_1 + 0x38) + 1,*(int *)(param_1 + 0x3c) + 1);
      SelectObject(hdc,pvVar4);
      SelectObject(hdc,pvVar5);
    }
    if ((*(HGDIOBJ *)(param_1 + 0xac) != (HGDIOBJ)0x0) &&
       (iVar1 = *(int *)(param_1 + 0x98), iVar1 != 0)) {
      iVar2 = *(int *)(param_1 + 0x38);
      uVar6 = *(uint *)(param_1 + 0x94);
      SelectObject(hdc,*(HGDIOBJ *)(param_1 + 0xac));
      pvVar4 = GetStockObject(8);
      pvStack_c = SelectObject(hdc,pvVar4);
      Rectangle(hdc,0,0,(uint)(iVar2 * iVar1) / uVar6 + 1,*(int *)(param_1 + 0x3c) + 1);
      SelectObject(hdc,pvStack_8);
      SelectObject(hdc,(HGDIOBJ)local_40._44_4_);
    }
    if (*(int *)(param_1 + 0x9c) != 0) {
      pvVar4 = *(HGDIOBJ *)(param_1 + 0xb4);
      if (pvVar4 == (HGDIOBJ)0x0) {
        pvVar4 = GetStockObject(7);
      }
      pvStack_4 = SelectObject(hdc,pvVar4);
      uVar6 = *(uint *)(param_1 + 0x94);
      uVar7 = 0;
      if (uVar6 != 0) {
        do {
          uVar6 = (uVar7 * *(int *)(param_1 + 0x38)) / uVar6;
          MoveToEx(hdc,uVar6,0,(LPPOINT)0x0);
          LineTo(hdc,uVar6,*(int *)(param_1 + 0x3c));
          uVar7 = uVar7 + *(int *)(param_1 + 0x9c);
          uVar6 = *(uint *)(param_1 + 0x94);
          pcVar8 = SelectObject_exref;
        } while (uVar7 < uVar6);
      }
      (*pcVar8)(hdc,pvStack_4);
    }
    return 0;
  }
  LVar3 = DefWindowProcA(*(HWND *)(param_1 + 0x18),param_2,param_3,param_4);
  return LVar3;
}

