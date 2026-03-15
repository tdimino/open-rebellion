
void __thiscall FUN_00410850(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  undefined2 extraout_var;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a5a0;
  local_c = ExceptionList;
  if (param_1 == (void *)0x0) {
    return;
  }
  ExceptionList = &local_c;
  FUN_00411710(param_1,local_18);
  local_4 = 0;
  FUN_005f2f50(local_24);
  local_4 = CONCAT31(local_4._1_3_,1);
  iVar1 = FUN_0040ffd0((int)local_18,local_24,
                       CONCAT22(extraout_var,*(undefined2 *)((int)this + 0x94)));
  if (iVar1 == 0) {
LAB_00410970:
    if (iVar1 == 3) goto LAB_00410975;
  }
  else {
    if (iVar1 != 3) {
      FUN_004109e0((int)this);
      FUN_005974c0();
      FUN_00597050();
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x38))();
      if (iVar1 == 1) {
        FUN_00406820();
      }
      if (iVar1 == 2) {
        FUN_00406800();
      }
      iVar2 = FUN_00597480();
      if (iVar2 == 0) {
        FUN_00410b10((int *)this);
      }
      else {
        FUN_00597340();
        FUN_00597360();
        *(undefined4 *)((int)this + 0x118) = 1;
        KillTimer(*(HWND *)((int)this + 0x18),0x191);
        KillTimer(*(HWND *)((int)this + 0x18),400);
        FUN_00597050();
        DAT_006b1208 = 0;
        iVar2 = FUN_00597480();
        *(undefined4 *)((int)this + 0x118) = 1;
        if (iVar2 == 0) {
          FUN_00401040(3);
        }
        else {
          FUN_00401040(0xe);
        }
      }
      goto LAB_00410970;
    }
LAB_00410975:
    if ((DAT_006b123c & 1) != 0) goto LAB_0041099e;
  }
  if (*(void **)((int)this + 0x180) != (void *)0x0) {
    FUN_00420b70(*(void **)((int)this + 0x180),(int)local_24,0);
  }
  if (iVar1 == 3) {
    DAT_006b123c = DAT_006b123c | 1;
  }
LAB_0041099e:
  local_4 = local_4 & 0xffffff00;
  FUN_005f2ff0(local_24);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return;
}

