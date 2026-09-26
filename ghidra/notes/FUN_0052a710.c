
uint __thiscall FUN_0052a710(void *this,uint *param_1,void *param_2)

{
  void *pvVar1;
  int iVar2;
  uint uVar3;
  bool bVar4;
  bool bVar5;
  uint local_4;
  
  local_4 = 1;
  bVar4 = false;
  bVar5 = false;
  pvVar1 = (void *)FUN_00504dc0(param_1);
  if (pvVar1 != (void *)0x0) {
    iVar2 = FUN_00528890(this,pvVar1);
    if (iVar2 == 0) {
      pvVar1 = FUN_004f5940((void *)(*(int *)((int)this + 0x54) + 0x18),param_1);
      bVar5 = pvVar1 != (void *)0x0;
      if (bVar5) {
        FUN_004f58e0((void *)(*(int *)((int)this + 0x54) + 0x18),param_1);
      }
    }
    else {
      pvVar1 = FUN_004f5940((void *)(*(int *)((int)this + 0x54) + 0x18),param_1);
      bVar4 = pvVar1 == (void *)0x0;
      if (bVar4) {
        FUN_004f44b0((void *)(*(int *)((int)this + 0x54) + 0x18),param_1,0);
      }
    }
  }
  if ((bVar4) || (bVar5)) {
    uVar3 = FUN_0052a9c0(this,param_2);
    local_4 = 0;
    if (uVar3 != 0) {
      iVar2 = FUN_0052a7d0(this,param_2);
      local_4 = 0;
      if (iVar2 != 0) {
        local_4 = FUN_0052aac0(this,param_2);
      }
    }
  }
  return local_4;
}

