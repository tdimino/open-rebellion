
void __thiscall FUN_004e4eb0(void *this,void *param_1)

{
  int *piVar1;
  void *pvVar2;
  void *pvStack_4;
  
  pvVar2 = param_1;
  pvStack_4 = this;
  FUN_005f5630((void *)((int)this + 0xc),param_1);
  param_1 = *(void **)((int)this + 4);
  FUN_005f4db0(pvVar2,&param_1);
  FUN_005f4db0(pvVar2,(int)this + 0x28);
  for (piVar1 = *(int **)((int)this + 8); piVar1 != (int *)0x0; piVar1 = (int *)piVar1[2]) {
    pvStack_4 = (void *)(**(code **)(*piVar1 + 0xc))();
    FUN_005f4db0(pvVar2,&pvStack_4);
    (**(code **)(*piVar1 + 8))(pvVar2);
  }
  return;
}

