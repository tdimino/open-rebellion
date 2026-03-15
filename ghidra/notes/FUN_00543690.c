
uint FUN_00543690(undefined4 *param_1,int *param_2,undefined4 *param_3)

{
  void *this;
  undefined4 *puVar1;
  int *piVar2;
  undefined4 *this_00;
  bool bVar3;
  undefined4 uVar4;
  undefined3 extraout_var;
  uint *puVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  uint uVar5;
  
  this_00 = param_3;
  piVar2 = param_2;
  puVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006472e0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = 0;
  *param_2 = 0;
  FUN_004ece80(param_3);
  param_1 = (undefined4 *)0x38000343;
  local_4 = 0;
  uVar4 = FUN_005066a0((uint *)&param_1);
  *puVar1 = uVar4;
  local_4 = 0xffffffff;
  FUN_00619730();
  this = (void *)*puVar1;
  uVar5 = (uint)(this != (void *)0x0);
  if (uVar5 != 0) {
    bVar3 = FUN_004f6b50(this,piVar2);
    uVar5 = CONCAT31(extraout_var,bVar3);
    if (uVar5 != 0) {
      puVar6 = FUN_004025b0((void *)*piVar2,(uint *)&param_1);
      local_4 = 1;
      FUN_004f26d0(this_00,puVar6);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return uVar5;
}

