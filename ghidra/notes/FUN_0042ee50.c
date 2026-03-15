
void __thiscall FUN_0042ee50(void *this,void *param_1)

{
  void *pvVar1;
  int *piVar2;
  void *pvVar3;
  void *pvStack_4;
  
  pvVar1 = param_1;
  pvStack_4 = this;
  FUN_005f4d90(param_1,(int)this + 0xc);
                    /* WARNING: Load size is inaccurate */
  (**(code **)(*this + 4))();
  FUN_005f4d90(pvVar1,&param_1);
  pvVar3 = (void *)0x0;
  if (param_1 != (void *)0x0) {
    do {
      FUN_005f4d90(pvVar1,&pvStack_4);
      piVar2 = (int *)FUN_0042efe0(this,pvStack_4);
      if (piVar2 != (int *)0x0) {
        piVar2[0x10] = *(int *)((int)this + 0x10);
        piVar2[0x11] = *(int *)((int)this + 0x14);
        (**(code **)(*piVar2 + 4))(pvVar1);
        FUN_005f5440(this,piVar2);
      }
      pvVar3 = (void *)((int)pvVar3 + 1);
    } while (pvVar3 < param_1);
  }
  return;
}

