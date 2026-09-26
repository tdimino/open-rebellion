
uint FUN_00555540(int *param_1,int *param_2)

{
  int *piVar1;
  uint uVar2;
  uint *puVar3;
  int iVar4;
  void *unaff_ESI;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649458;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  FUN_004ece30(&param_2);
  local_4 = 0;
  uVar2 = (**(code **)(*param_1 + 0xc))(&param_2);
  if (uVar2 != 0) {
    puVar3 = FUN_004ece40((uint *)&param_1);
    if (puVar3 != (uint *)0x0) {
      iVar4 = FUN_00504dc0((uint *)&param_1);
      uVar2 = (uint)(iVar4 != 0);
      *piVar1 = iVar4;
    }
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = unaff_ESI;
  return uVar2;
}

