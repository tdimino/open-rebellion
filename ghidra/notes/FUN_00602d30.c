
void __thiscall FUN_00602d30(void *param_1,HDC param_2,undefined1 param_3)

{
  int iVar1;
  LPCSTR lpchText;
  void *this;
  LPRECT lprc;
  UINT format;
  
  SelectPalette(param_2,DAT_006be5a0,0);
  if ((*(uint *)((int)param_1 + 0xc4) & 2) != 0) {
    *(undefined4 *)((int)param_1 + 0xf8) = 0;
    param_3 = 2;
  }
  switch(param_3) {
  case 0:
    goto switchD_00602d7c_caseD_0;
  case 1:
    if (*(int *)((int)param_1 + 0xf8) != 0) {
      SetTextColor(param_2,*(COLORREF *)((int)param_1 + 0xb8));
      this = *(void **)((int)param_1 + 0x98);
      goto LAB_00602e30;
    }
    goto LAB_00602dfc;
  case 2:
  case 0x40:
    this = *(void **)((int)param_1 + 0x9c);
    if (this == (void *)0x0) {
      this = *(void **)((int)param_1 + 0x94);
    }
    SetTextColor(param_2,*(COLORREF *)((int)param_1 + 0xc0));
    goto LAB_00602e30;
  default:
    goto switchD_00602d7c_caseD_3;
  case 4:
    break;
  }
  *(uint *)((int)param_1 + 0xc4) = *(uint *)((int)param_1 + 0xc4) | 4;
switchD_00602d7c_caseD_0:
  if ((*(uint *)((int)param_1 + 0xc4) & 4) == 0) {
    if ((*(uint *)((int)param_1 + 0xc4) & 0x40) == 0) {
LAB_00602dfc:
      SetTextColor(param_2,*(COLORREF *)((int)param_1 + 0xb4));
      this = *(void **)((int)param_1 + 0x94);
    }
    else {
      SetTextColor(param_2,*(COLORREF *)((int)param_1 + 0xc0));
      this = *(void **)((int)param_1 + 0x9c);
      if (this != (void *)0x0) goto LAB_00602e38;
      this = *(void **)((int)param_1 + 0x94);
    }
LAB_00602e30:
    if (this != (void *)0x0) goto LAB_00602e38;
  }
  else {
    SetTextColor(param_2,*(COLORREF *)((int)param_1 + 0xbc));
    this = *(void **)((int)param_1 + 0xa0);
    if (this == (void *)0x0) {
      this = *(void **)((int)param_1 + 0x94);
      goto LAB_00602e30;
    }
LAB_00602e38:
    if ((*(uint *)((int)param_1 + 0xc4) & 0x80) == 0) {
      FUN_005fcc30(*(void **)((int)param_1 + 0xa4),*(uint **)((int)param_1 + 0xa8),0,0,
                   *(int *)((int)param_1 + 0x10c),*(uint *)((int)param_1 + 0x110),(undefined4 *)0x0,
                   0);
      FUN_005fd0f0(this,*(uint **)((int)param_1 + 0xa8),0,0);
      this = *(void **)((int)param_1 + 0xa8);
    }
    else if ((*(uint *)((int)param_1 + 0xc4) & 0x20000) != 0) {
      FUN_005fcc30(*(void **)((int)param_1 + 0xa4),*(uint **)((int)param_1 + 0xa8),0,0,
                   *(int *)((int)param_1 + 0x10c),*(uint *)((int)param_1 + 0x110),(undefined4 *)0x0,
                   0);
      FUN_005fcc30(this,*(uint **)((int)param_1 + 0xa8),0,0,0,0,(undefined4 *)0x0,0);
      this = *(void **)((int)param_1 + 0xa8);
    }
    FUN_005fc140(this,param_2,'@',0xcc0020,0,0,0,0,0,0);
  }
  if (((*(int *)((int)param_1 + 200) != 0) &&
      (iVar1 = FUN_00583c40(*(int *)((int)param_1 + 200)), iVar1 != 0)) &&
     ((*(byte *)((int)param_1 + 0xc4) & 0x20) != 0)) {
    FUN_006002c0(param_1,param_2);
    SetBkMode(param_2,1);
    lprc = (LPRECT)((int)param_1 + 0xcc);
    format = 1;
    iVar1 = -1;
    lpchText = (LPCSTR)FUN_00583c40(*(int *)((int)param_1 + 200));
    DrawTextA(param_2,lpchText,iVar1,lprc,format);
    FUN_006002f0(param_1,param_2);
  }
  *(uint *)((int)param_1 + 0xc4) = *(uint *)((int)param_1 + 0xc4) & 0xfffeffff;
switchD_00602d7c_caseD_3:
  return;
}

