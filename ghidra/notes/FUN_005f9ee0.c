
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __thiscall FUN_005f9ee0(void *this,int param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined4 *unaff_EDI;
  int *piVar3;
  
  uVar1 = 3;
  if (param_1 != 0) {
    _DAT_006be3ac = *(int **)(param_1 + 0x1c);
    param_1 = 0;
    piVar3 = _DAT_006be3ac;
    iVar2 = Ordinal_1(_DAT_006be3ac,&param_1,0);
    if (unaff_EDI != (undefined4 *)0x0) {
      iVar2 = (**(code **)*unaff_EDI)(unaff_EDI,&DAT_0066e488,(int)this + 0x108);
      (**(code **)(*piVar3 + 8))(piVar3);
    }
    if ((iVar2 == 0) && (*(int *)((int)this + 0x108) != 0)) {
      return 0;
    }
    _DAT_006be3ac = (int *)0x0;
    *(undefined4 *)((int)this + 0x108) = 0;
    uVar1 = 5;
  }
  return uVar1;
}

