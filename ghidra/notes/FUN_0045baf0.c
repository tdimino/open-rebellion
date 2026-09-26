
void __thiscall FUN_0045baf0(void *this,uint *param_1,int param_2,int param_3)

{
  int *this_00;
  int *piVar1;
  BOOL BVar2;
  
  this_00 = (int *)FUN_00604500((void *)((int)this + 0x6c),(ushort)((short)*param_1 << 6) | 3);
  if (this_00 != (int *)0x0) {
    piVar1 = FUN_004f3220(*(int *)(*(int *)((int)this + 0x194) + 0x9c),param_1);
    if ((*(byte *)(piVar1 + 0x14) & 8) == 0) {
      if ((-1 < param_3) && (this_00[0x26] != param_3)) {
        FUN_0060e440(this_00,param_3);
      }
      if ((this_00[0x25] != param_2) && (0 < param_2)) {
        FUN_0060e400(this_00,param_2,0);
      }
      BVar2 = IsWindowVisible((HWND)this_00[6]);
      if (((BVar2 == 0) && ((*(byte *)(piVar1 + 9) & 0xc0) != 0)) &&
         ((*(byte *)(piVar1 + 0x22) & 1) != 0)) {
        (**(code **)(*this_00 + 4))(5);
        return;
      }
    }
    else {
      (**(code **)(*this_00 + 8))();
    }
  }
  return;
}

