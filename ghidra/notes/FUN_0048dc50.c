
void __fastcall FUN_0048dc50(int param_1)

{
  int iVar1;
  uint uVar2;
  bool bVar3;
  ushort uVar4;
  int iVar5;
  int iVar6;
  void *this;
  int *piVar7;
  int *piVar8;
  uint *puVar9;
  uint uVar10;
  uint local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635708;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar5 = FUN_004f3e50(*(int *)(param_1 + 0x20));
  iVar6 = FUN_004f3eb0(*(int *)(param_1 + 0x20));
  this = (void *)FUN_004f3e50(2);
  local_10 = (void *)FUN_004f3eb0(2);
  local_14 = 0x32000242;
  local_4 = 0;
  if (*(int *)(param_1 + 0x68) == 0x32000242) {
    iVar1 = *(int *)(param_1 + 0x6c);
    FUN_00619730();
    if (((iVar1 != 0x35000281) || (iVar5 == 0)) || (bVar3 = true, iVar6 == 0)) goto LAB_0048dcf1;
  }
  else {
LAB_0048dcf1:
    bVar3 = false;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (!bVar3) {
    *(undefined4 *)(param_1 + 0x40) = 0;
    ExceptionList = local_c;
    return;
  }
  FUN_0060b9d0((void *)(param_1 + 0x44),0x71a0,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0
               ,(undefined4 *)0x0);
  FUN_0060b9d0((void *)(param_1 + 0x50),0x71a1,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0
               ,(undefined4 *)0x0);
  if ((*(byte *)(iVar5 + 0xac) & 1) == 0) {
    if (*(short *)(iVar5 + 0x94) != 0) {
      uVar10 = 0x71a4;
      goto LAB_0048de87;
    }
    if ((*(byte *)(iVar6 + 0xac) & 1) == 0) {
      uVar10 = 0x71a5;
      goto LAB_0048de87;
    }
  }
  else {
    bVar3 = false;
    if ((this != (void *)0x0) && (local_10 != (void *)0x0)) {
      piVar7 = FUN_0042d170(local_10,&local_14);
      local_4 = 1;
      piVar8 = FUN_0042d170(this,&local_10);
      iVar5 = *piVar8;
      iVar1 = *piVar7;
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00619730();
      if (iVar5 == iVar1) {
        iVar5 = FUN_004f3f10(2);
        puVar9 = FUN_0042d170(this,&local_10);
        local_4 = 2;
        piVar7 = FUN_004f3000(2,puVar9);
        local_4 = 0xffffffff;
        FUN_00619730();
        if ((piVar7 != (int *)0x0) && (iVar5 != 0)) {
          puVar9 = FUN_004025b0(*(void **)(iVar5 + 0x1c),&local_14);
          uVar10 = piVar7[0x1d];
          uVar2 = *puVar9;
          FUN_00619730();
          if (uVar10 == uVar2) {
            bVar3 = true;
          }
        }
      }
    }
    if (bVar3) {
      uVar10 = 0x71a2;
    }
    else {
      uVar10 = 0x71a3;
    }
LAB_0048de87:
    FUN_0060b9d0((void *)(param_1 + 0x50),uVar10,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0,(undefined4 *)0x0);
  }
  if ((*(byte *)(iVar6 + 0xac) & 1) == 0) {
    if (*(short *)(iVar6 + 0x94) == 0) goto LAB_0048decc;
    uVar10 = 0x71a7;
  }
  else {
    uVar10 = 0x71a6;
  }
  FUN_0060b9d0((void *)(param_1 + 0x50),uVar10,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0
               ,(undefined4 *)0x0);
LAB_0048decc:
  *(undefined2 *)(param_1 + 0x2e) = 0x422;
  *(undefined2 *)(param_1 + 0x30) = 0x46d;
  uVar4 = FUN_004c4990(&DAT_0000011e);
  *(ushort *)(param_1 + 0x32) = uVar4;
  ExceptionList = local_c;
  return;
}

