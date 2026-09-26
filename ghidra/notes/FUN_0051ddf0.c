
bool __fastcall FUN_0051ddf0(int param_1)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  
  if (*(int *)(param_1 + 0xc0) != 0) {
    iVar2 = FUN_00516ef0(*(int *)(param_1 + 0xc0));
    iVar3 = FUN_0051eb90(param_1);
    if ((iVar3 == 0) || (iVar2 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_00517270(*(int *)(param_1 + 0xc0));
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_0051eb90(param_1);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_0051e050(param_1);
    if ((iVar2 != 0) && (bVar1)) {
      return true;
    }
  }
  return false;
}

