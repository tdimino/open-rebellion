
void __fastcall FUN_0060cac0(int param_1)

{
  int iVar1;
  LONG LVar2;
  void *pvVar3;
  int iVar4;
  int iVar5;
  tagPOINT local_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656b2b;
  local_c = ExceptionList;
  local_14.x = 0;
  local_14.y = 0;
  if (*(int *)(param_1 + 0xe8) == 0) {
    ExceptionList = &local_c;
    ClientToScreen(*(HWND *)(param_1 + 0x18),&local_14);
    LVar2 = local_14.x;
    iVar4 = *(int *)(param_1 + 0xa4) * *(int *)(param_1 + 0xa8) + 4;
    iVar5 = *(int *)(param_1 + 0x3c) + local_14.y;
    if (*(int *)(param_1 + 0xcc) < iVar4 + iVar5) {
      iVar1 = *(int *)(param_1 + 0xc4);
      iVar5 = local_14.y - iVar4;
      if (local_14.y - iVar4 < iVar1) {
        iVar4 = local_14.y - iVar1;
        iVar5 = iVar1;
      }
    }
    iVar1 = *(int *)(param_1 + 0x38);
    pvVar3 = (void *)FUN_00618b70(0xb4);
    uStack_4 = 0;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_0060cc50(pvVar3,*(undefined4 *)(param_1 + 0x1c),param_1,LVar2,iVar5,iVar1,iVar4,
                            *(undefined4 *)(param_1 + 0xb0));
    }
    uStack_4 = 0xffffffff;
    *(void **)(param_1 + 0xe8) = pvVar3;
    if (pvVar3 != (void *)0x0) {
      *(undefined4 *)((int)pvVar3 + 0x24) = 0x3e9;
      *(undefined4 *)(*(int *)(param_1 + 0xe8) + 0xb0) = *(undefined4 *)(param_1 + 0xe4);
      FUN_005ffce0(*(void **)(param_1 + 0xe8),1);
      InvalidateRect(*(HWND *)(*(int *)(param_1 + 0xe8) + 0x18),(RECT *)0x0,0);
    }
  }
  ExceptionList = local_c;
  return;
}

