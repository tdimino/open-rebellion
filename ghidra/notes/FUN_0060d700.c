
int * __thiscall FUN_0060d700(void *this,int *param_1,int param_2,int param_3)

{
  HWND hWndNewParent;
  undefined2 uVar1;
  int iVar2;
  undefined2 extraout_var;
  
  iVar2 = FUN_00604540((void *)((int)this + 0x6c),param_1[6]);
  if (iVar2 == 0) {
    hWndNewParent = *(HWND *)((int)this + 0x18);
    if (*(HWND *)(param_1[8] + 0x18) != hWndNewParent) {
      SetParent((HWND)param_1[6],hWndNewParent);
    }
    FUN_006030c0(param_1,8);
    iVar2 = FUN_00603550(param_1,4);
    if (iVar2 == 0) {
      uVar1 = FUN_006034a0(param_1,1);
      FUN_00603150(param_1,4,CONCAT22(extraout_var,uVar1));
    }
    if (hWndNewParent != (HWND)0x0) {
      (**(code **)(*param_1 + 4))(5);
    }
    FUN_005f4f10((void *)((int)this + 0x6c),(int)param_1);
    if ((param_2 != 0) || (*(int *)((int)this + 0x94) == 0)) {
      FUN_0060d7e0(this,param_1[9],param_3);
    }
    return param_1;
  }
  return (int *)0x0;
}

