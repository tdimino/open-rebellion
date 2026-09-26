
void __thiscall FUN_0042aaf0(void *this,uint *param_1,int *param_2,uint *param_3,int *param_4)

{
  int iVar1;
  int *piVar2;
  void *pvVar3;
  int iVar4;
  int *local_4;
  
  local_4 = param_2;
  if (param_2 == (int *)0x0) {
    local_4 = FUN_004f2d10(*(int *)((int)this + 0x9c),param_1);
  }
  iVar1 = thunk_FUN_005f5060((int)this + 0x478);
  FUN_0042b330(this,param_3,param_4,1);
  for (; iVar1 != 0; iVar1 = *(int *)(iVar1 + 0x10)) {
    piVar2 = (int *)FUN_00604500((void *)((int)this + 0x6c),*(uint *)(iVar1 + 0x18));
    if (piVar2 != (int *)0x0) {
      pvVar3 = FUN_004f5940(piVar2 + 0x45,param_1);
      if ((pvVar3 != (void *)0x0) ||
         ((param_3 != (uint *)0x0 &&
          (pvVar3 = FUN_004f5940(piVar2 + 0x45,param_3), pvVar3 != (void *)0x0)))) {
        (**(code **)(*piVar2 + 0x60))(param_1,local_4,param_3,param_4);
        iVar4 = FUN_004291d0(this,piVar2[9]);
        if (iVar4 != 0) {
          FUN_00428d70(this,piVar2);
        }
      }
    }
  }
  return;
}

