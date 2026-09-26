
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

uint __thiscall FUN_005aacb0(void *this,float param_1)

{
  bool bVar1;
  undefined4 *puVar2;
  undefined3 extraout_var;
  void *pvVar3;
  uint uVar4;
  float unaff_retaddr;
  
  if (*(int *)((int)this + 0x8b4) == 1) {
    puVar2 = (undefined4 *)FUN_0059ed70();
    *puVar2 = 0;
    FUN_005ae1f0((int)this);
    *(undefined4 *)((int)this + 0x8b4) = 2;
    return 0;
  }
  if (*(int *)((int)this + 0x8b4) == 2) {
    puVar2 = (undefined4 *)FUN_0059ed70();
    *puVar2 = 0;
    bVar1 = FUN_005a6d60(this,param_1 * _DAT_0066c230);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      return 0;
    }
    *(undefined4 *)((int)this + 0x8b4) = 4;
    puVar2 = (undefined4 *)FUN_0059ed70();
    *puVar2 = 1;
  }
  FUN_005ada10(this,param_1);
  if (*(int *)((int)this + 0x8c8) != 0) {
    pvVar3 = (void *)FUN_0059ed70();
    FUN_005a0820(pvVar3);
    (**(code **)(*(int *)this + 0x7c))(0);
    return (uint)(*(int *)((int)this + 0xa08) == 1);
  }
  uVar4 = (**(code **)(*(int *)this + 0x9c))(param_1 * _DAT_0066c230);
  *(float *)((int)this + 0x8f0) = unaff_retaddr + *(float *)((int)this + 0x8f0);
  if (uVar4 != 0) {
    *(undefined4 *)((int)this + 0x8c4) = 1;
  }
  return uVar4;
}

