
void __thiscall FUN_0042abd0(void *this,uint *param_1,int *param_2)

{
  int iVar1;
  int *piVar2;
  void *pvVar3;
  int iVar4;
  
  if (param_2 == (int *)0x0) {
    param_2 = FUN_004f2d10(*(int *)((int)this + 0x9c),param_1);
  }
  if (param_2 != (int *)0x0) {
    FUN_0042b330(this,param_1,param_2,1);
    for (iVar1 = thunk_FUN_005f5060((int)this + 0x478); iVar1 != 0; iVar1 = *(int *)(iVar1 + 0x10))
    {
      piVar2 = (int *)FUN_00604500((void *)((int)this + 0x6c),*(uint *)(iVar1 + 0x18));
      if ((piVar2 != (int *)0x0) &&
         (pvVar3 = FUN_004f5940(piVar2 + 0x45,param_1), pvVar3 != (void *)0x0)) {
        (**(code **)(*piVar2 + 100))(param_1,param_2);
        iVar4 = FUN_004291d0(this,piVar2[9]);
        if (iVar4 != 0) {
          FUN_00428d70(this,piVar2);
        }
      }
    }
  }
  return;
}

