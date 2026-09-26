
uint __thiscall FUN_004f6e60(void *this,int param_1,void *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined *puVar2;
  int iVar3;
  uint uVar4;
  void *pvVar5;
  void *pvVar6;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063fd38;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_0053a000((int)this);
  uVar4 = CONCAT31(extraout_var,bVar1);
  if (uVar4 != 0) {
    iVar3 = param_1;
    puVar2 = FUN_004f62d0((int)this);
    iVar3 = FUN_005f3390(puVar2,iVar3);
    if (iVar3 != 0) {
      puVar2 = FUN_004f62d0((int)this);
      FUN_005f2f90(local_18,(int)puVar2);
      local_4 = 0;
      uVar4 = FUN_004fc680((int)this);
      if (uVar4 != 0) {
        FUN_005f3090(*(void **)((int)this + 0x34),param_1);
        pvVar6 = param_2;
        pvVar5 = (void *)FUN_00539fd0(this,1);
        FUN_004f9ba0(pvVar5,pvVar6);
        pvVar6 = param_2;
        pvVar5 = (void *)FUN_00539fd0(this,2);
        FUN_004f9ba0(pvVar5,pvVar6);
        (**(code **)(*(int *)this + 0xec))(local_18,*(undefined4 *)((int)this + 0x34),param_2);
      }
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
    }
  }
  ExceptionList = local_c;
  return uVar4;
}

