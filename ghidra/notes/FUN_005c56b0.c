
void __fastcall FUN_005c56b0(void *param_1)

{
  void *this;
  undefined4 uVar1;
  uint *puVar2;
  uint uVar3;
  undefined1 uVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653bdb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (this == (void *)0x0) {
    puVar2 = (uint *)0x0;
  }
  else {
    uVar4 = 2;
    uVar3 = 0x515;
    uVar1 = FUN_006037f0(6);
    puVar2 = (uint *)FUN_005fbd20(this,uVar1,uVar3,uVar4);
  }
  local_4 = 0xffffffff;
  FUN_005ff2d0(param_1,puVar2,0,0);
  ExceptionList = local_c;
  return;
}

