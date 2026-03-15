
undefined4 __thiscall FUN_0060e0a0(void *this,HDC param_1,int *param_2)

{
  if (*(void **)((int)this + 8) == (void *)0x0) {
    return 0;
  }
  FUN_005fcc30(*(void **)((int)this + 8),*(uint **)((int)this + 0xc),0,0,*param_2,param_2[1],
               (undefined4 *)0x0,0);
                    /* WARNING: Load size is inaccurate */
  FUN_005fd0f0(*this,*(uint **)((int)this + 0xc),0,0);
  FUN_005fc140(*(void **)((int)this + 0xc),param_1,'@',0xcc0020,*param_2,param_2[1],0,0,0,0);
  *(int *)((int)this + 0x18) = *param_2;
  *(int *)((int)this + 0x1c) = param_2[1];
  return 1;
}

