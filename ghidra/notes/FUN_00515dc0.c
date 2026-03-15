
bool __cdecl FUN_00515dc0(uint *param_1,int param_2,void *param_3)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  bool bVar3;
  
  iVar2 = thunk_FUN_00506e40();
  bVar3 = iVar2 != 0;
  if (iVar2 != 0) {
    bVar1 = FUN_00545950(param_1,param_2,param_3);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
  }
  if (param_2 != 0) {
    iVar2 = FUN_00506e10();
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    if (iVar2 != 0) {
      *(undefined4 *)(iVar2 + 0x70) = 1;
    }
  }
  return bVar3;
}

