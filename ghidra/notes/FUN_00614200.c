
int __thiscall FUN_00614200(void *this,undefined4 param_1,int param_2)

{
  int iVar1;
  
  DAT_006be6ec = param_1;
  iVar1 = -0x7789ff7e;
  if (param_2 == 0) {
    iVar1 = DirectDrawEnumerateA(&LAB_00615220,this);
  }
  if (DAT_006be6d4 == 0) {
    iVar1 = DirectDrawCreate(0,&DAT_006be6d8,0);
    if (iVar1 == 0) {
      iVar1 = (**(code **)*DAT_006be6d8)(DAT_006be6d8,&DAT_0066e4b8,&DAT_006be6d4);
    }
    else {
      DAT_006be6d4 = 0;
    }
    if (iVar1 != 0) {
      DAT_006be6d4 = 0;
    }
  }
  FUN_00614530(iVar1,(HWND)0x0);
  return iVar1;
}

