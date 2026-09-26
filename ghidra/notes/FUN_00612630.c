
int __thiscall FUN_00612630(void *this,uint *param_1,int param_2)

{
  uint *puVar1;
  bool bVar2;
  void *pvVar3;
  undefined3 extraout_var;
  int iVar4;
  
  puVar1 = param_1;
  iVar4 = -0x7fffbffb;
  pvVar3 = (void *)FUN_00612bd0((void *)((int)this + 0x40),*param_1);
  if (pvVar3 == (void *)0x0) {
    if (*puVar1 == 0) {
      pvVar3 = (void *)FUN_00612bd0((void *)((int)this + 0x40),1);
      if (pvVar3 != (void *)0x0) {
        bVar2 = FUN_00613cb0(pvVar3);
        if (CONCAT31(extraout_var,bVar2) == 1) {
          param_1 = *(uint **)((int)pvVar3 + 0x54);
          pvVar3 = FUN_00612770(this,(int *)&param_1);
          if (pvVar3 != (void *)0x0) {
            *puVar1 = *(uint *)((int)pvVar3 + 0x18);
            iVar4 = FUN_00613b50(pvVar3,param_2);
          }
        }
        else {
          iVar4 = FUN_00613b50(pvVar3,param_2);
          *puVar1 = *(uint *)((int)pvVar3 + 0x18);
        }
      }
    }
  }
  else {
    iVar4 = FUN_00613b50(pvVar3,param_2);
  }
  if (iVar4 == 0) {
    *(int *)((int)this + 0x1c) = *(int *)((int)this + 0x1c) + 1;
  }
  return iVar4;
}

