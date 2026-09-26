
int __thiscall FUN_005710b0(void *this,int *param_1,int *param_2)

{
  bool bVar1;
  uint uVar2;
  int iVar3;
  int *piVar4;
  int iVar5;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064c1d8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  local_4 = 0;
  uVar2 = (**(code **)(*param_1 + 4))();
  if ((uVar2 < 0x30) || (0x3b < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar4 = (int *)0x0;
  if (bVar1) {
    piVar4 = param_1;
  }
  iVar3 = FUN_00505190((uint *)((int)this + 0xa8));
  if ((iVar3 == 0) || (!bVar1)) {
    iVar5 = 0;
  }
  else {
    iVar5 = 1;
  }
  if ((iVar3 != 0) && (piVar4 != (int *)0x0)) {
    iVar3 = FUN_0055c7c0((int)(short)piVar4[0x23],(int)*(short *)(iVar3 + 0x8c),param_2);
    if ((iVar3 == 0) || (iVar5 == 0)) {
      iVar5 = 0;
    }
    else {
      iVar5 = 1;
    }
  }
  ExceptionList = pvStack_c;
  return iVar5;
}

