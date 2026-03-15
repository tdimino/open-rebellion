
uint FUN_005434b0(undefined4 *param_1,int *param_2,undefined4 *param_3)

{
  undefined4 *puVar1;
  int *piVar2;
  undefined4 *this;
  bool bVar3;
  void *this_00;
  undefined3 extraout_var;
  uint *puVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  uint uVar4;
  
  this = param_3;
  piVar2 = param_2;
  puVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647278;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = 0;
  *param_2 = 0;
  FUN_004ece80(param_3);
  this_00 = (void *)FUN_00507010();
  *puVar1 = this_00;
  uVar4 = (uint)(this_00 != (void *)0x0);
  if (uVar4 != 0) {
    bVar3 = FUN_004f6b50(this_00,piVar2);
    uVar4 = CONCAT31(extraout_var,bVar3);
    if (uVar4 != 0) {
      puVar5 = FUN_004025b0((void *)*piVar2,(uint *)&param_1);
      local_4 = 0;
      FUN_004f26d0(this,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return uVar4;
}

