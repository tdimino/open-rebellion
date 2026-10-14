
void __thiscall FUN_0059ca90(void *this,uint param_1,int param_2)

{
  void *pvVar1;
  
  pvVar1 = (void *)FUN_00604500((void *)((int)this + 0x6c),param_1);
  if (pvVar1 != (void *)0x0) {
    if (param_2 == 1) {
      FUN_00603710(pvVar1);
      return;
    }
    FUN_00603780(pvVar1);
  }
  return;
}

