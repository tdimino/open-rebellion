
undefined4 __cdecl
FUN_004fcf50(int param_1,uint *param_2,undefined4 *param_3,int *param_4,undefined4 *param_5)

{
  void *this;
  undefined4 uVar1;
  int iVar2;
  
  *param_2 = 0;
  *param_3 = 0;
  *param_4 = 0;
  uVar1 = 1;
  *param_5 = 0;
  if (param_1 == 1) {
    iVar2 = 1;
  }
  else if (param_1 == 2) {
    iVar2 = 2;
  }
  else {
    iVar2 = 0;
  }
  this = (void *)FUN_0051ce00();
  if (this != (void *)0x0) {
    uVar1 = FUN_0051dce0(this,iVar2,param_2,param_3,param_4,param_5);
  }
  return uVar1;
}

