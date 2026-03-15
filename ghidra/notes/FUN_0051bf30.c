
int FUN_0051bf30(int *param_1,int param_2,int param_3,void *param_4)

{
  int iVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  int *this;
  
  this = (int *)0x0;
  iVar4 = 1;
  if (0 < param_2) {
    do {
      iVar1 = thunk_FUN_005f5060(param_3);
      if (iVar1 != 0) {
        piVar2 = FUN_004f26e0((int *)(iVar1 + 0x24));
        this = param_1;
        if (piVar2 != (int *)0x0) {
          this = FUN_004f7d50(param_1,(uint *)(iVar1 + 0x24),(uint)param_1[9] >> 6 & 3,param_4,1,1);
          if ((this == (int *)0x0) || (iVar4 == 0)) {
            iVar4 = 0;
          }
          else {
            iVar4 = 1;
          }
        }
      }
      for (iVar1 = *(int *)(iVar1 + 0x10); iVar1 != 0; iVar1 = *(int *)(iVar1 + 0x10)) {
        if (this != (int *)0x0) {
          iVar3 = FUN_004f7cf0(this,1,(uint *)(iVar1 + 0x24),(uint)this[9] >> 6 & 3,param_4,1,1);
          if ((iVar3 == 0) || (iVar4 == 0)) {
            iVar4 = 0;
          }
          else {
            iVar4 = 1;
          }
        }
      }
      param_2 = param_2 + -1;
    } while (param_2 != 0);
  }
  return iVar4;
}

