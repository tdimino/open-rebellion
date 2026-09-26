
void __cdecl FUN_005cc1b0(int param_1)

{
  uint *puVar1;
  undefined4 *puVar2;
  int *piVar3;
  void *this;
  int *piVar4;
  int iVar5;
  void *unaff_ESI;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654351;
  pvStack_c = ExceptionList;
  puVar1 = *(uint **)(param_1 + 4);
  ExceptionList = &pvStack_c;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    ExceptionList = &pvStack_c;
    puVar2 = (undefined4 *)FUN_00618b70(0x48);
    local_4 = 0;
    if (puVar2 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar2);
    }
  }
  local_4 = 0xffffffff;
  piVar3 = (int *)FUN_005a6460(DAT_006bc470,*puVar1);
  if (piVar3 == (int *)0x0) {
    this = (void *)FUN_00618b70(0xbc);
    local_4 = 1;
    if (this == (void *)0x0) {
      piVar3 = (int *)0x0;
    }
    else {
      piVar3 = FUN_005cc070(this,puVar1[1],*puVar1);
    }
    local_4 = 0xffffffff;
    piVar4 = (int *)FUN_005aaf90();
    (**(code **)(*piVar4 + 0x4c))();
  }
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)FUN_00618b70(0x48);
    local_4 = 2;
    if (puVar2 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar2);
    }
    local_4 = 0xffffffff;
  }
  iVar5 = FUN_005a6340(DAT_006bc470,puVar1[1]);
  *(int *)(iVar5 + 0x174) = piVar3[6];
  (**(code **)(*piVar3 + 8))();
  FUN_005caf70(piVar3,*(undefined4 *)(iVar5 + 0x30),*(undefined4 *)(iVar5 + 0x34),
               *(undefined4 *)(iVar5 + 0x38),*(undefined4 *)(iVar5 + 0x3c));
  piVar3 = (int *)FUN_005aaf90();
  (**(code **)(*piVar3 + 0x38))();
  ExceptionList = unaff_ESI;
  return;
}

