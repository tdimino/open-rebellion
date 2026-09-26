
int * __fastcall FUN_004bbc50(int *param_1)

{
  bool bVar1;
  void *pvVar2;
  int iVar3;
  uint uVar4;
  int *piVar5;
  
  piVar5 = (int *)0x0;
  if (((uint)param_1[0xb] >> 0x18 < 0x30) || (0x3f < (uint)param_1[0xb] >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    pvVar2 = FUN_004f5940((void *)param_1[0x10],(uint *)(param_1 + 0xb));
    if ((pvVar2 != (void *)0x0) && ((*(uint *)((int)pvVar2 + 0x30) & 0x74000f1) == 0)) {
      if (*(uint *)((int)pvVar2 + 0x38) == param_1[10]) {
        if ((*(uint *)((int)pvVar2 + 0x30) & 0x7000000) == 0) {
          iVar3 = (**(code **)(*param_1 + 0x18))();
          if (iVar3 != 6) {
            iVar3 = (**(code **)(*param_1 + 0x1c))();
            piVar5 = FUN_004bbf50(param_1,iVar3);
            return piVar5;
          }
        }
      }
      else {
        uVar4 = (uint)param_1[10] >> 0x18;
        if ((uVar4 < 0x90) || (0x97 < uVar4)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          piVar5 = FUN_004bc040(param_1,(uint *)(param_1 + 10));
        }
      }
    }
  }
  return piVar5;
}

