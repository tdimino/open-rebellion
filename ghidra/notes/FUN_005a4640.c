
void * __thiscall FUN_005a4640(void *this,uint param_1)

{
  void *pvVar1;
  undefined4 uVar2;
  undefined4 *puVar3;
  int iVar4;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652503;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0059e880(this);
  *(undefined ***)this = &PTR_FUN_0066bbf8;
  local_4 = 0;
  FUN_005c0e60();
  *(undefined4 *)((int)this + 0x60) = 0;
  *(undefined4 *)((int)this + 100) = 0;
  *(uint *)((int)this + 0x54) = param_1;
  pvVar1 = (void *)FUN_0059ed70();
  uVar2 = FUN_0059f5f0(pvVar1,param_1);
  *(undefined4 *)((int)this + 0x58) = uVar2;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar3 = (undefined4 *)FUN_00618b70(0x48);
    local_4._0_1_ = 1;
    if (puVar3 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar3);
    }
    local_4 = (uint)local_4._1_3_ << 8;
  }
  uVar2 = FUN_005a6470(DAT_006bc470,*(uint *)((int)this + 0x58));
  *(undefined4 *)((int)this + 0x5c) = uVar2;
  FUN_005c0f00(uVar2);
  iVar4 = FUN_0059ed70();
  FUN_0059ef60(iVar4);
  uVar2 = 0;
  pvVar1 = (void *)FUN_0059ed70();
  FUN_0051c9b0(pvVar1,uVar2);
  ExceptionList = local_c;
  return this;
}

