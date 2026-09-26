
void __thiscall FUN_0042f510(void *this,void *param_1)

{
  void *pvVar1;
  int *piVar2;
  void *pvVar3;
  int iStack_8;
  undefined4 uStack_4;
  
  pvVar1 = param_1;
  FUN_005f4d90(param_1,(int)this + 0xc);
  (**(code **)(*(int *)this + 4))();
  FUN_005f4d90(pvVar1,&param_1);
  pvVar3 = (void *)0x0;
  if (param_1 != (void *)0x0) {
    do {
      FUN_005f4d90(pvVar1,&uStack_4);
      FUN_005f4d90(pvVar1,&iStack_8);
      piVar2 = FUN_0042f830(this,uStack_4,iStack_8);
      if (piVar2 != (int *)0x0) {
        piVar2[0x19] = *(int *)((int)this + 0x10);
        piVar2[0x1a] = *(int *)((int)this + 0x14);
        (**(code **)(*piVar2 + 4))(pvVar1);
        FUN_005f5440(this,piVar2);
      }
      pvVar3 = (void *)((int)pvVar3 + 1);
    } while (pvVar3 < param_1);
  }
  return;
}

