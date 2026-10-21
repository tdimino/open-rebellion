
void __thiscall FUN_00497380(void *param_1,int param_2,uint *param_3,uint *param_4)

{
  bool bVar1;
  void *pvVar2;
  int *piVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006364db;
  pvStack_c = ExceptionList;
  if (param_2 != 0x302) {
    ExceptionList = &pvStack_c;
    FUN_004c5020(param_1,param_2,param_3);
    ExceptionList = pvStack_c;
    return;
  }
  ExceptionList = &pvStack_c;
  pvVar2 = (void *)FUN_00618b70(0x1c);
  uStack_4 = 0;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_004ecc70(pvVar2,param_3);
  }
  uStack_4 = 0xffffffff;
  piVar3 = FUN_004f2d10(*(int *)((int)param_1 + 0x20),param_3);
  bVar1 = false;
  if ((piVar3 != (int *)0x0) && ((*(byte *)(piVar3 + 0x14) & 4) == 0)) {
    bVar1 = true;
  }
  if (!bVar1) {
    if ((*param_4 >> 0x18 < 0x90) || (0x97 < *param_4 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      param_1 = (void *)((int)param_1 + 0x88);
      goto LAB_00497471;
    }
  }
  param_1 = (void *)((int)param_1 + 0x80);
LAB_00497471:
  FUN_004f57b0(param_1,pvVar2);
  ExceptionList = pvStack_c;
  return;
}

