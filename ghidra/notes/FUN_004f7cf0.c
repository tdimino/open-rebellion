
int __thiscall
FUN_004f7cf0(void *this,int param_1,uint *param_2,undefined4 param_3,void *param_4,int param_5,
            int param_6)

{
  int *piVar1;
  int iVar2;
  
  iVar2 = 1;
  if (0 < param_1) {
    do {
      piVar1 = FUN_004f7d50(this,param_2,param_3,param_4,param_5,param_6);
      if ((piVar1 == (int *)0x0) || (iVar2 == 0)) {
        iVar2 = 0;
      }
      else {
        iVar2 = 1;
      }
      param_1 = param_1 + -1;
    } while (param_1 != 0);
  }
  return iVar2;
}

