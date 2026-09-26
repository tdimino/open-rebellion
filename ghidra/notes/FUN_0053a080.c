
int __thiscall FUN_0053a080(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  int iVar3;
  undefined4 uVar4;
  uint uVar5;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  pvVar1 = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00646278;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  iVar2 = FUN_005405a0(this,param_1);
  if (iVar2 != 0) {
    *(undefined4 *)((int)this + 0x2c) = 0;
    FUN_005f4d90(pvVar1,&local_10);
    pvVar1 = local_10;
    if (local_10 != (void *)0x0) {
      iVar3 = (**(code **)(*(int *)this + 4))();
      uVar5 = (uint)param_1 & 0xffffff | iVar3 << 0x18;
      param_1 = (void *)(uVar5 ^ ((uint)pvVar1 ^ uVar5) & 0xffffff);
      uStack_4 = 0;
      uVar4 = FUN_0051cab0((uint *)&param_1);
      *(undefined4 *)((int)this + 0x2c) = uVar4;
      uStack_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return iVar2;
}

