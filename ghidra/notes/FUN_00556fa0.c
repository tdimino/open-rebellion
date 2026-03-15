
uint __thiscall FUN_00556fa0(void *this,int *param_1,void *param_2)

{
  int *piVar1;
  bool bVar2;
  undefined3 extraout_var;
  uint uVar3;
  undefined4 local_28 [5];
  undefined4 local_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649780;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar2 = FUN_004f9190(this,param_1,param_2);
  uVar3 = CONCAT31(extraout_var,bVar2);
  if (uVar3 != 0) {
    piVar1 = (int *)((int)this + 0x58);
    FUN_00540230(piVar1,DAT_006bb404);
    FUN_00540270(piVar1,DAT_006bb41c);
    FUN_004fd450(local_28,(int)param_2);
    local_4 = 0;
    local_14 = FUN_004fd2f0();
    uVar3 = FUN_0053fa60(0x38f,1,this,piVar1,local_28);
    local_4 = 0xffffffff;
    FUN_004fd4d0(local_28);
    if (uVar3 != 0) {
      piVar1 = (int *)((int)this + 0x60);
      FUN_00540230(piVar1,DAT_006bb4b8);
      FUN_00540270(piVar1,DAT_006bb4a8);
      FUN_004fd450(local_28,(int)param_2);
      local_4 = 1;
      local_14 = FUN_004fd2f0();
      uVar3 = FUN_0053fa60(0x390,1,this,piVar1,local_28);
      local_4 = 0xffffffff;
      FUN_004fd4d0(local_28);
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

