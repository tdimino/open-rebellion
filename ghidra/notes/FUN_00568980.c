
undefined4 __thiscall FUN_00568980(void *this,void *param_1)

{
  void *pvVar1;
  int *piVar2;
  
  pvVar1 = param_1;
  FUN_005f4db0(param_1,(int)this + 0x10);
  param_1 = (void *)FUN_005cf920((int)this);
  FUN_005f4db0(pvVar1,&param_1);
  for (piVar2 = (int *)FUN_00583c40((int)this); piVar2 != (int *)0x0; piVar2 = (int *)piVar2[2]) {
    param_1 = (void *)(**(code **)(*piVar2 + 0xc))();
    FUN_005f4db0(pvVar1,&param_1);
    (**(code **)(*piVar2 + 8))(pvVar1);
  }
  return 1;
}

