
void __thiscall FUN_0048a3b0(void *this,int *param_1)

{
  int iVar1;
  void *this_00;
  char *pcVar2;
  int *piVar3;
  undefined4 uVar4;
  int iVar5;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00634e9b;
  pvStack_c = ExceptionList;
  piVar3 = (int *)0x0;
  ExceptionList = &pvStack_c;
  if (param_1 != (int *)0x0) {
    ExceptionList = &pvStack_c;
    iVar1 = (**(code **)(*param_1 + 0xc))();
    if (iVar1 == 0x11) {
      piVar3 = param_1;
    }
  }
  if (piVar3[8] != *(int *)this) {
    this_00 = (void *)FUN_00618b70(100);
    uStack_4 = 0;
    if (this_00 == (void *)0x0) {
      piVar3 = (int *)0x0;
    }
    else {
      iVar1 = *(int *)this;
      iVar5 = 1;
      pcVar2 = (char *)FUN_00583c40((int)(piVar3 + 0x11));
      piVar3 = FUN_00498e90(this_00,pcVar2,iVar1,iVar5);
    }
    uStack_4 = 0xffffffff;
    if (piVar3[0x10] != 0) {
      uVar4 = FUN_0041d120();
      FUN_005f5600(piVar3,uVar4);
      FUN_0048a060(this,piVar3);
    }
  }
  ExceptionList = pvStack_c;
  return;
}

