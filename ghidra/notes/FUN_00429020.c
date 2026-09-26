
void __thiscall FUN_00429020(void *this,int param_1)

{
  undefined4 *puVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  undefined4 *puVar5;
  int iVar6;
  tagRECT local_44;
  tagRECT tStack_34;
  undefined1 local_24 [4];
  uint local_20;
  undefined4 local_1c;
  uint local_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c338;
  local_c = ExceptionList;
  if (param_1 == 0) {
    return;
  }
  if (0xc < param_1) {
    return;
  }
  ExceptionList = &local_c;
  FUN_00610b20(local_24,0,0,0,0,0,0);
  local_4 = 0;
  local_14 = local_14 & 0xfffffff9 | 1;
  local_1c = 7;
  puVar1 = (undefined4 *)((int)this + (param_1 + 0x19) * 0x1c);
  local_20 = 0x265 - (*(int *)((int)this + 0x9c) != 1) & 0xffff;
  iVar2 = FUN_00604500((void *)((int)this + 0x6c),
                       *(uint *)((int)this + (param_1 + 0x19) * 0x1c + 0x18));
  if (iVar2 == 0) {
    ExceptionList = local_c;
    return;
  }
  CopyRect(&local_44,(RECT *)(puVar1 + 2));
  SetRect(&tStack_34,*(int *)(iVar2 + 0x28),*(int *)(iVar2 + 0x2c),
          *(int *)(iVar2 + 0x38) + *(int *)(iVar2 + 0x28),
          *(int *)(iVar2 + 0x3c) + *(int *)(iVar2 + 0x2c));
  FUN_00610c30((int)local_24);
  FUN_00428940(*(HWND *)((int)this + 0x18),&tStack_34.left,&local_44.left);
  iVar3 = *(int *)((int)this + 0x458);
  iVar6 = 0;
  if (0 < iVar3) {
    piVar4 = (int *)((int)this + 0x428);
    do {
      if (*piVar4 == puVar1[6]) {
        if (iVar3 <= iVar6) goto LAB_00429159;
        puVar5 = (undefined4 *)((int)this + iVar6 * 4 + 0x428);
        goto LAB_00429142;
      }
      iVar6 = iVar6 + 1;
      piVar4 = piVar4 + 1;
    } while (iVar6 < iVar3);
  }
  goto LAB_00429176;
  while( true ) {
    *puVar5 = puVar5[1];
    iVar3 = *(int *)((int)this + 0x458);
    puVar5 = puVar5 + 1;
    if (iVar3 <= iVar6) break;
LAB_00429142:
    iVar6 = iVar6 + 1;
    if (iVar6 == iVar3) break;
  }
LAB_00429159:
  *(int *)((int)this + 0x458) = *(int *)((int)this + 0x458) + -1;
  InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)(puVar1 + 2),0);
LAB_00429176:
  puVar5 = (undefined4 *)*puVar1;
  puVar1[6] = 0;
  puVar1[1] = 0;
  if (puVar5 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
    *puVar1 = 0;
  }
  SendMessageA(*(HWND *)((int)this + 0x18),0x467,*(WPARAM *)(iVar2 + 0x18),0);
  ExceptionList = local_c;
  return;
}

