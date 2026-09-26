
void FUN_004158e0(int param_1,int param_2)

{
  undefined4 *puVar1;
  int *piVar2;
  int in_stack_00000018;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0062abb2;
  local_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x38);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4._0_1_ = 1;
  if (puVar1 == (undefined4 *)0x0) {
    piVar2 = (int *)0x0;
  }
  else {
    piVar2 = FUN_004210c0(puVar1);
  }
  local_4._0_1_ = 0;
  if (piVar2 != (int *)0x0) {
    piVar2[9] = param_2;
    piVar2[8] = param_1;
    FUN_005f2f90(&stack0xffffffdc,(int)&stack0x0000000c);
    FUN_00416560((int)piVar2);
    piVar2[0xd] = in_stack_00000018;
    puVar1 = (undefined4 *)FUN_00618b70(0x1c);
    local_4._0_1_ = 2;
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      puVar1 = FUN_0051c300(puVar1);
    }
    local_4 = (uint)local_4._1_3_ << 8;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_0051c410(puVar1,piVar2);
      if (puVar1[1] != 0) {
        FUN_005970d0((undefined4 *)puVar1[6],puVar1[5],4);
        FUN_00597170();
      }
      (**(code **)*puVar1)();
      (**(code **)*piVar2)();
    }
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0((undefined4 *)&stack0x0000000c);
  ExceptionList = local_c;
  return;
}

