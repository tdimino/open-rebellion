
void __fastcall FUN_004abd30(int param_1)

{
  undefined4 *puVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  void *this;
  uint *puVar4;
  int iVar5;
  int iVar6;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063847b;
  local_c = ExceptionList;
  puVar4 = (uint *)0x0;
  if ((*(int **)(param_1 + 0x18) != (int *)0x0) &&
     (ExceptionList = &local_c, bVar2 = FUN_005fc040(*(int **)(param_1 + 0x18)),
     CONCAT31(extraout_var,bVar2) != 0)) {
    iVar5 = *(int *)(param_1 + 0x68) - *(int *)(param_1 + 0x60);
    iVar3 = FUN_005fc0f0(*(int **)(param_1 + 0x18));
    this = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (this != (void *)0x0) {
      puVar4 = FUN_005fbda0(this,iVar5,iVar3,*(uint **)(param_1 + 0x18));
    }
    local_4 = 0xffffffff;
    iVar3 = FUN_005fc0e0(*(int **)(param_1 + 0x18));
    iVar6 = 0;
    local_10 = iVar5 / iVar3 + 1;
    if (0 < local_10) {
      do {
        FUN_005fcc30(*(void **)(param_1 + 0x18),puVar4,iVar6,0,0,0,(undefined4 *)0x0,0);
        iVar6 = iVar6 + iVar3;
        local_10 = local_10 + -1;
      } while (local_10 != 0);
    }
    puVar1 = *(undefined4 **)(param_1 + 0x18);
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
    }
    *(uint **)(param_1 + 0x18) = puVar4;
  }
  ExceptionList = local_c;
  return;
}

