
bool __thiscall FUN_005688d0(void *this,void *param_1)

{
  void *pvVar1;
  int *piVar2;
  bool bVar3;
  int local_8;
  int local_4;
  
  pvVar1 = param_1;
  bVar3 = true;
  FUN_005f5ee0((int)this);
  FUN_005f4d90(param_1,(int)this + 0x10);
  FUN_005f4d90(param_1,&local_8);
  param_1 = (void *)0x0;
  if (0 < local_8) {
    do {
      FUN_005f4d90(pvVar1,&local_4);
      piVar2 = (int *)FUN_0051f730(local_4);
      bVar3 = piVar2 != (int *)0x0;
      if (bVar3) {
        (**(code **)(*piVar2 + 4))(pvVar1);
        FUN_00583c50(this,(int)piVar2);
      }
      param_1 = (void *)((int)param_1 + 1);
    } while (bVar3 && (int)param_1 < local_8);
  }
  return bVar3;
}

