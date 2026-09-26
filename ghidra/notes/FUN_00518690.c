
uint __cdecl FUN_00518690(int *param_1,int *param_2)

{
  int *piVar1;
  int *piVar2;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar2 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642ad8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_2 = 0;
  param_2 = (int *)0x0;
  uVar3 = FUN_005185c0(param_1,(int *)&param_2);
  if ((uVar3 != 0) && (param_2 != (int *)0x0)) {
    piVar1 = (int *)param_2[7];
    uVar3 = 0;
    if (piVar1 != (int *)0x0) {
      local_4 = 0;
      uVar3 = (**(code **)(*piVar1 + 4))();
      if ((uVar3 < 0x80) || (0x8f < uVar3)) {
        uVar3 = 0;
      }
      else {
        uVar3 = 1;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      if (uVar3 != 0) {
        *piVar2 = (int)piVar1;
      }
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

