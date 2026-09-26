
void * __thiscall FUN_004ebb90(void *this,void *param_1)

{
  void *this_00;
  uint *puVar1;
  undefined4 *puVar2;
  int iVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063e9f6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = FUN_004025b0(param_1,(uint *)&param_1);
  local_4 = 0;
  FUN_004ecc70(this,puVar1);
  local_4._0_1_ = 2;
  FUN_00619730();
  FUN_005f4950((void *)((int)this + 0x24),0);
  local_4._0_1_ = 3;
  FUN_005f4950((void *)((int)this + 0x28),0);
  local_4 = CONCAT31(local_4._1_3_,4);
  *(undefined ***)this = &PTR_FUN_0065ca10;
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x38) = 0xffffffff;
  *(undefined4 *)((int)this + 0x3c) = 0;
  puVar2 = (undefined4 *)FUN_00509620(this_00,&param_1);
  *(undefined4 *)((int)this + 0x2c) = *puVar2;
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x34) = 0;
  puVar2 = (undefined4 *)((int)this + 0x40);
  for (iVar3 = 0x16; iVar3 != 0; iVar3 = iVar3 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  *(undefined4 *)((int)this + 0x98) = 0;
  *(undefined4 *)((int)this + 0x1c) = 1;
  ExceptionList = local_c;
  return this;
}

