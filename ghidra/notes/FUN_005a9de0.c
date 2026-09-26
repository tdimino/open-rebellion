
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __thiscall FUN_005a9de0(void *this,float param_1)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  int *piVar4;
  int *this_00;
  
  timeGetTime();
  if (((*(int *)((int)this + 0x9ec) == 1) && (iVar1 = FUN_005a9330(), iVar1 != 0)) &&
     (iVar2 = FUN_005ba420(iVar1), iVar2 != 0)) {
    FUN_005b8800(iVar1 + 0x1c);
  }
  piVar3 = (int *)(**(code **)(*(int *)((int)this + 0x988) + 0x10))();
joined_r0x005a9e28:
  do {
    if (piVar3 == (int *)0x0) {
      return 1;
    }
    piVar4 = (int *)(**(code **)(*piVar3 + 8))();
    this_00 = (int *)(**(code **)(*piVar3 + 0x10))();
    iVar1 = FUN_005ad870(this_00);
    piVar3 = piVar4;
  } while (iVar1 == 0);
  FUN_005b6320(this_00,param_1 * _DAT_0066c26c);
  iVar1 = (**(code **)(*this_00 + 0x20))();
  if (iVar1 == 0) goto code_r0x005a9e6c;
  goto LAB_005a9e8c;
code_r0x005a9e6c:
  (**(code **)(*this_00 + 0x9c))(param_1);
  if (this_00[0x49] == 0) {
    FUN_005b8800((int)(this_00 + 0x106));
LAB_005a9e8c:
    if (this_00[0x49] == 0) {
      (**(code **)(*this_00 + 0x88))();
    }
  }
  goto joined_r0x005a9e28;
}

