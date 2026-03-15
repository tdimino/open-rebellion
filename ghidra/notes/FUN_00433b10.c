
void * __thiscall FUN_00433b10(void *this,undefined4 param_1,void *param_2)

{
  undefined4 uVar1;
  void *this_00;
  uint *puVar2;
  undefined4 *puVar3;
  int iVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d261;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar2 = FUN_004025b0(param_2,(uint *)&param_2);
  local_4 = 0;
  FUN_004ecc70(this,puVar2);
  local_4._0_1_ = 2;
  FUN_00619730();
  FUN_005f4950((void *)((int)this + 0x28),0);
  local_4._0_1_ = 3;
  FUN_005f4950((void *)((int)this + 0x2c),0);
  local_4._0_1_ = 4;
  FUN_005f4950((void *)((int)this + 0x30),0);
  *(undefined4 *)((int)this + 0x1c) = param_1;
  local_4 = CONCAT31(local_4._1_3_,5);
  *(undefined ***)this = &PTR_FUN_00658d70;
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined4 *)((int)this + 0x44) = 0;
  *(undefined4 *)((int)this + 0x38) = 0;
  puVar3 = (undefined4 *)FUN_00509620(this_00,&param_2);
  uVar1 = *puVar3;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x34) = uVar1;
  *(undefined4 *)((int)this + 0x40) = 0;
  puVar3 = (undefined4 *)((int)this + 0x48);
  for (iVar4 = 0x3c; iVar4 != 0; iVar4 = iVar4 + -1) {
    *puVar3 = 0;
    puVar3 = puVar3 + 1;
  }
  ExceptionList = local_c;
  return this;
}

