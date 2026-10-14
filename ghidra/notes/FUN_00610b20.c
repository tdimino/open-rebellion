
void __thiscall
FUN_00610b20(void *this,uint param_1,undefined4 param_2,undefined4 param_3,uint param_4,uint param_5
            ,undefined4 param_6)

{
  *(undefined4 *)((int)this + 4) = param_3;
  *(undefined4 *)((int)this + 8) = param_2;
  *(undefined4 *)((int)this + 0xc) = param_6;
  *(undefined ***)this = &PTR_LAB_00658518;
  *(uint *)((int)this + 0x10) =
       param_1 & 7 ^ *(uint *)((int)this + 0x10) & 0xffffff80 |
       (param_4 & 1 | (param_5 & 1) << 1) << 3;
  return;
}

