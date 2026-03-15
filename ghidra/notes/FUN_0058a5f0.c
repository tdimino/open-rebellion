
undefined4 __thiscall FUN_0058a5f0(void *this,undefined4 param_1)

{
  void *this_00;
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  int aiStack_24 [3];
  int aiStack_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_0064fc70;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*(int *)((int)this + 0x50) + 4))();
  (**(code **)(*(int *)((int)this + 0x58) + 4))();
  this_00 = (void *)((int)this + 8);
  FUN_005894d0(aiStack_18,this_00,(int)this + 0x50);
  iStack_4 = 0;
  iVar1 = FUN_00587fc0(this_00,aiStack_18,param_1);
  FUN_005894d0(aiStack_24,this_00,(int)this + 0x58);
  iStack_4._0_1_ = 1;
  iVar2 = FUN_00587fe0(this_00,aiStack_24,param_1);
  if ((iVar2 == 0) || (iVar1 == 0)) {
    uVar3 = 0;
  }
  else {
    uVar3 = 1;
  }
  iStack_4 = (uint)iStack_4._1_3_ << 8;
  FUN_00589510(aiStack_24);
  iStack_4 = 0xffffffff;
  FUN_00589510(aiStack_18);
  ExceptionList = pvStack_c;
  return uVar3;
}

